class CreateQuotesTable < Sequel::Migration

    def up
        execute <<-END_SQL
            CREATE TABLE quotes (
                quote_id integer primary key,
                quote varchar not null,
                user varchar not null,
                posted timestamp,
                poster varchar not null,
                linked_quote_id integer
            )
        END_SQL
    end

    def down
        execute "DROP TABLE quotes"
    end

end


