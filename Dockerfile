# Use an Alpine Linux base image
FROM i686/ubuntu
USER root

# Install necessary tools
RUN apt update && \
    apt -y install wget ca-certificates make gcc

RUN mkdir /usr/bin/php
# Download and install PHP 1
RUN wget -O /usr/bin/php_tar https://museum.php.net/php1/php-108.tar.gz --no-check-certificate && \
    tar -zxvf /usr/bin/php_tar -C /usr/bin/php --strip-components=1
RUN ["chmod", "o+x", "/usr/bin/php"]
RUN ["chmod", "+x", "-R", "/var/log"]
RUN cd usr/bin/php && make

EXPOSE 8080

#RUN adduser admin
#RUN echo "secret_new_root_password"

RUN ls /usr/bin/php
ENTRYPOINT ["/usr/bin/php"]
CMD ["--version --verbose"]
