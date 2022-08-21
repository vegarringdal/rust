

playing around with connecting to oracledb and reading data

# How to get started (with this app)

-   `install rust`
-   `git clone repo...`

-   start oracle express

    -   create folder called `oracleExpress` on c disk
    -   create folder called `setup` & `startup`
    -   add this file `user_testdb.sql` under `setup` with content

              alter session set "_ORACLE_SCRIPT"=true;

              CREATE USER TESTDB
              IDENTIFIED by TESTDB
              QUOTA UNLIMITED ON users;

              GRANT ALL PRIVILEGES TO TESTDB;

    -   Then run: docker run --name oracle_db -p 1522:1521 -e ORACLE_PWD=admin -v
        c:/oracleExpress/data:/opt/oracle/oradata -v c:/oracleExpress/setup:/opt/oracle/scripts/setup -v
        c:/oracleExpress/startup:/opt/oracle/scripts/startup vegarringdal/oracledb-express-21.3.0:1.0.0

One liner:

-   `docker run --name oracle_db -p 1522:1521 -e ORACLE_PWD=admin -v c:/oracleExpress/data:/opt/oracle/oradata -v c:/oracleExpress/setup:/opt/oracle/scripts/setup -v c:/oracleExpress/startup:/opt/oracle/scripts/startup vegarringdal/oracledb-express-21.3.0:1.0.0`
-   next time you should be able to start with `docker start oracle_db`


- add table/dummy data
```sql
CREATE TABLE T_PERSON (
    ID               NUMBER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    FIRSTNAME        VARCHAR2(50) NOT NULL,
    LASTNAME         VARCHAR2(50) NOT NULL,
    BORN             DATE NOT NULL,
    CHILDREN         NUMBER,
    COMPANY_ID       NUMBER,      
    CREATED          DATE DEFAULT SYSDATE,
    CREATED_BY       VARCHAR2(100),
    MODIFIED         DATE DEFAULT SYSDATE,
    MODIFIED_BY      VARCHAR2(100),
    FOREIGN KEY (COMPANY_ID) REFERENCES T_COMPANY(ID)
);

CREATE TRIGGER TR_PERSON_AUDIT BEFORE
    INSERT OR UPDATE ON T_PERSON
    FOR EACH ROW
BEGIN
    IF INSERTING THEN
        :NEW.CREATED_BY := F_GET_CURRENT_USER();
        :NEW.MODIFIED_BY := F_GET_CURRENT_USER();
    END IF;

    IF UPDATING THEN
        :NEW.MODIFIED_BY := F_GET_CURRENT_USER();
        :NEW.MODIFIED := SYSDATE;
    END IF;

END;
```

```sql
BEGIN
    FOR C IN 1..10000 LOOP
        INSERT INTO T_PERSON (
            FIRSTNAME,
            LASTNAME,
            BORN
        ) VALUES (
            'FIRST_' || C,
            'LAST_' || C,
            sysdate - C
        );

    END LOOP;
END;
```