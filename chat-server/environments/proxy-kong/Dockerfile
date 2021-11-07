FROM kong:alpine

USER root

RUN set -x && \
    apk add --update libintl && \
    apk add --virtual build_deps gettext &&  \
    cp /usr/bin/envsubst /usr/local/bin/envsubst && \
    apk del build_deps

COPY kong.template.yml /kong.template.yml
COPY start.sh /start.sh
RUN chmod +x /start.sh

EXPOSE 8000 8443 8001 8444

ENTRYPOINT ["/start.sh"]

CMD ["kong", "docker-start"]
