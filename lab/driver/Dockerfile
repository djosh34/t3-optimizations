# The phone side of the lab: Playwright WebKit with the driver's Playwright
# 1.64.1, which matches the browsers in the base image.
FROM mcr.microsoft.com/playwright:v1.64.1-noble
RUN mkdir -p /opt/driver \
  && cd /opt/driver \
  && npm init -y >/dev/null \
  && npm install --no-audit --no-fund playwright@1.64.1
