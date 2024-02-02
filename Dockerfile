# Use the official PHP image as base
FROM php:7.4-apache

# Set the working directory in the container
WORKDIR /var/www/html

# Copy only necessary files for the application
COPY index.php .
#
## Set the PORT environment variable
#ENV PORT=8080

FROM php:7-apache
RUN sed -i 's/Listen 80/Listen 8080}/' /etc/apache2/ports.conf
# Expose the port that Cloud Run will use
EXPOSE 8080

# Start Apache server in the foreground
CMD ["apache2-foreground"]