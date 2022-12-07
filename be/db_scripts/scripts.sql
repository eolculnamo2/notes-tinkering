CREATE TABLE users (
   id varchar(255) UNIQUE NOT NULL,
   email varchar(255) UNIQUE NOT NULL,
    first_name varchar(255) NOT NULL,
   last_name varchar(255) NOT NULL,
   password varchar(255) NOT NULL
);

CREATE TABLE notes (
   id varchar(255) UNIQUE NOT NULL,
   title varchar(255) NOT NULL,
    body varchar(3000) DEFAULT NULL,
   category varchar(255) NOT NULL,
   created integer NOT NULL,
   last_modified integer NOT NULL,
   user_fk varchar(255) NOT NULL
);

