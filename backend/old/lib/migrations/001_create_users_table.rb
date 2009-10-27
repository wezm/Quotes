class CreateUsersTable < Sequel::Migration

    def up
        execute <<-END_SQL
            CREATE TABLE users (
                username varchar primary key,
                name varchar not null,
                password varchar not null,
                last_posted timestamp,
                favourite_quote_id integer
            )
        END_SQL
    end

    def down
        execute "DROP TABLE users"
    end

end

