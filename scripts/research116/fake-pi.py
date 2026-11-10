#!/usr/bin/env python3
"""A local Pi protocol fixture. No model, network, tool or subagent calls."""
import json
import sys
import threading
import time
import uuid

if '--version' in sys.argv:
    print('1.4.0')
    sys.exit(0)
lock = threading.Lock()
streaming = False
session = '/tmp/research116/fake-session-' + str(uuid.uuid4()) + '.jsonl'
model = dict(provider='fake', id='research116', name='research116', reasoning=False, contextWindow=100000, maxTokens=10000, cost=dict(input=0,output=0,cacheRead=0,cacheWrite=0), input=['text'])

def emit(v):
    with lock:
        print(json.dumps(v), flush=True)

def reply():
    global streaming
    streaming = True
    emit(dict(type='agent_start'))
    emit(dict(type='message_start', message=dict(role='assistant')))
    text = ''
    for i in range(120):
        delta = f'STREAM116 {i:03}. This is a synthetic streaming reply.\n\n'
        text += delta
        emit(dict(type='message_update', assistantMessageEvent=dict(type='text_delta', contentIndex=0, delta=delta)))
        time.sleep(.25)
    emit(dict(type='message_end', message=dict(role='assistant', content=[dict(type='text',text=text)], stopReason='stop')))
    streaming = False
    emit(dict(type='agent_end', messages=[], willRetry=False))
    emit(dict(type='agent_settled'))

for line in sys.stdin:
    try:
        r=json.loads(line)
    except ValueError:
        continue
    tag=r.get('type')
    data={}
    if tag=='get_state':
        data=dict(sessionFile=session,sessionId='research116',isStreaming=streaming,isCompacting=False,thinkingLevel='off',model=model,steeringMode='one-at-a-time',followUpMode='one-at-a-time',autoCompactionEnabled=False,messageCount=0,pendingMessageCount=0)
    elif tag=='get_available_models':data=dict(models=[model])
    elif tag=='get_commands':data=dict(commands=[])
    elif tag=='get_entries':data=dict(entries=[],leafId=None)
    elif tag=='get_messages':data=dict(messages=[])
    elif tag=='set_model':data=model
    elif tag in ['new_session','switch_session']:data=dict(cancelled=False)
    emit(dict(type='response',id=r.get('id'),command=tag,success=True,data=data))
    if tag=='prompt':threading.Thread(target=reply,daemon=True).start()
