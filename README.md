# movie_ratings

A platform to rate & share movies with others

 - **TypeScript** + **React** GUI
 - **Rust** + **actix-web** & **juniper** on-prem GraphQL Server
 - **postgres** DB
 - **docker-compose** 


## Contributing

### Getting set up.


1. Install
  - [docker](https://www.docker.com/)
  - [docker-compose](https://docs.docker.com/compose/)
2. In the root of the repo, run
   ```bash
      docker-compose up -d;
   ```
   to create Node, Rust & Postgres containers.



### BE

1. We use [diesel](https://diesel.rs/) as an ORM.  Use that to generate all migrations / seeds.  [Here's a quick intro guide](https://diesel.rs/guides/getting-started).  Note that the `diesel_cli` comes pre-installed on the Docker image.
