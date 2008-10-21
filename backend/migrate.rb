require 'rubygems'
require 'sequel'

db = Sequel.sqlite('quotes.db')

Sequel::Migrator.apply(db, 'lib/migrations')

