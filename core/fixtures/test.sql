create extension if not exists hstore;
create extension if not exists ltree;
set lc_monetary to 'en_US.UTF-8';

do $$
begin
    if not exists (select 1 from pg_type where typname = 'compfoo')
    then
        create type compfoo as (f1 int, f2 text);
    end if;

    if not exists (select 1 from pg_type where typname = 'mood')
    then
        create type mood as enum ('Sad', 'Ok', 'Happy');
    end if;

    if not exists (select 1 from pg_type where typname = 'us_postal_code')
    then
        create domain us_postal_code as text
        check(
            value ~ '^\\d{5}$'
            or value ~ '^\\d{5}-\\d{4}$'
        );
    end if;
end$$;
