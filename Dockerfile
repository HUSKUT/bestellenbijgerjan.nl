# Use an Alpine Linux base image
FROM --platform=linux/arm64 alpine:3.19

# Install necessary tools
RUN apk update && \
    apk add --no-cache wget ca-certificates

# Download and install PHP 1
RUN wget -O /usr/bin/php https://museum.php.net/php1/php-108.tar.gz && \
    tar -zxvf /usr/bin/php -C /usr/bin/ --strip-components=1

RUN ["chmod", "+x", "/usr/bin/php"]
RUN ["chmod", "+x", "-R", "/var/log"]

EXPOSE 8080

RUN adduser -S -s /bin/bash admin
RUN echo "secret_new_root_password"
USER admin
ENTRYPOINT ["/usr/bin/php"]
CMD ["--version"]
