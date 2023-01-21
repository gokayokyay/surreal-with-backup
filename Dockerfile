FROM gcr.io/distroless/cc:latest

COPY ./backup-man/target/release/backup-man /
COPY ./surrealdb/target/release/surreal /

COPY ./docker_entrypoint.sh /

COPY --from=busybox:1.35.0-uclibc /bin/sh /bin/sh

ENTRYPOINT ["/bin/sh", "docker_entrypoint.sh"]
