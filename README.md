# plitedb

plitedb is a lightweight, fully embedded NoSQL database inspired by SQLite and DynamoDB.
It features a simple query language (ironic :D) and thrives with transactional data.
plitedb aims to fill the niche of a lightweight, dead simple database for K/V storage, with some other nicities provided through various operations.

# Features / Roadmap
- [ ] Query language
    - [x] Tokenization
    - [x] Parsing
    - [ ] Evaluation
- [ ] Engine
    - [ ] Reading and writing
    - [ ] Disk serialization
    - [ ] Rollback / recovery
    - [ ] Write-Ahead Logging
- [ ] Operations
    - [ ] GET
        - [ ] Hash key
        - [ ] Hash key + sort key
    - [ ] PUT
    - [ ] QUERY
    - [ ] UPDATE
        - [ ] Fixed SET values
        - [ ] Lambda SET functions