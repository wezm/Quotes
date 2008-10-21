class CreateRatingsTable < Sequel::Migration

    def up
        execute <<-END_SQL
            CREATE TABLE ratings (
                quote_id integer not null,
                rater varchar not null,
                rating integer not null
            )
        END_SQL
    end

    def down
        execute "DROP TABLE ratings"
    end

end


