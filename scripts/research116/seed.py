#!/usr/bin/env python3
"""Only seed a stopped throwaway stock t3 under /tmp, as in research #90."""
import json
import pathlib
import sqlite3
import sys
import subprocess

base = pathlib.Path(sys.argv[1]).resolve()
assert str(base).startswith('/tmp/'), 'throwaway instances only'
c = sqlite3.connect(base / 'userdata/statev2.sqlite')
project = c.execute('select project_id from projection_projects limit 1').fetchone()[0]
now = '2026-11-10T10:00:00.000Z'

statements = ['BEGIN;']

def insert(table, row):
    values = ["NULL" if v is None else "'" + str(v).replace("'", "''") + "'" for v in row.values()]
    statements.append(f'insert or replace into {table} ({",".join(row)}) values ({",".join(values)});')

for thread_id in ['research116-long', 'research116-stream']:
    thread = dict(id=thread_id, projectId=project, title=thread_id, providerInstanceId='pi',
                  modelSelection=dict(instanceId='pi', model='fake/research116'), runtimeMode='approval-required',
                  interactionMode='default', branch=None, worktreePath=None, activeProviderThreadId=None,
                  lineage=dict(parentThreadId=None, relationshipToParent=None, rootThreadId=thread_id),
                  forkedFrom=None, createdAt=now, updatedAt=now, archivedAt=None, deletedAt=None,
                  createdBy='user', creationSource='mobile', settledOverride=None, settledAt=None, lastVisitedAt=None)
    insert('orchestration_v2_projection_threads', dict(thread_id=thread_id, project_id=project,
           title=thread_id, default_provider='pi', provider_instance_id='pi', runtime_mode='approval-required',
           interaction_mode='default', created_at=now, updated_at=now, payload_json=json.dumps(thread)))
    for i in range(600):
        role = 'user' if i % 2 == 0 else 'assistant'
        message_id = f'{thread_id}-message-{i:04}'
        text = f'MARKER {i:04} {role}\n\n'
        if role == 'user':
            text += 'Explain this fake scrolling experiment. ' * (1 + i % 4)
        else:
            text += '\n\n'.join(f'Paragraph {p}. ' + 'Synthetic history for the scrolling test. ' * (2 + i % 6) for p in range(3 + i % 9))
            if i % 5 == 1:
                text += '\n\n```typescript\n' + '\n'.join(f'const row{n} = "fake line {n}";' for n in range(12)) + '\n```'
        item = dict(id=f'{thread_id}-item-{i:04}', threadId=thread_id, runId=None, nodeId=None,
                    providerThreadId=None, providerTurnId=None, nativeItemRef=None, parentItemId=None,
                    ordinal=i, status='completed', title=None, startedAt=now, completedAt=now,
                    updatedAt=now, type=f'{role}_message', messageId=message_id, text=text)
        if role == 'user':
            item.update(createdBy='user', creationSource='mobile', inputIntent='turn_start', attachments=[])
        else:
            item.update(streaming=False)
        insert('orchestration_v2_projection_turn_items', dict(turn_item_id=item['id'], thread_id=thread_id,
               ordinal=i, type=item['type'], status='completed', updated_at=now, payload_json=json.dumps(item)))
        insert('orchestration_v2_turn_item_positions', dict(thread_id=thread_id, turn_item_id=item['id'], ordinal=i))
        message = dict(id=message_id, threadId=thread_id, runId=None, nodeId=None, role=role, text=text,
                       streaming=False, createdAt=now, updatedAt=now, attachments=[], createdBy='user' if role == 'user' else 'agent', creationSource='mobile' if role == 'user' else 'provider')
        insert('orchestration_v2_projection_messages', dict(message_id=message_id, thread_id=thread_id, role=role,
               streaming=0, created_at=now, updated_at=now, payload_json=json.dumps(message)))
c.close()
statements.append('COMMIT;')
subprocess.run(['bun', '-e', 'import {Database} from "bun:sqlite"; const d=new Database(process.argv[1]); d.exec(await Bun.stdin.text());', str(base / 'userdata/statev2.sqlite')], input='\n'.join(statements), text=True, check=True)
print('Seeded two synthetic 600-message threads. No provider was started.')
