Quotes
======

History
-------

When I was in uni I wrote a PHP app for my group of friends to be able to log quotes from each other. These aren't so much profound quotes but humorous ones. The PHP app managed the user profiles and quotes in a series of text files as a database was not available on the uni server that hosted it.

This app is the result of digging up those quote files, converting them to an SQLite db and wrapping a Sinatra app around it.

Install/Run
-----------

__Note:__ My development environment for this app is ruby 1.9.2 (on Mac OS X).

Install the necessary gem via Bubdler:

    bundle install

### Run ###

For development, using shotgun:

    bundle exec shotgun config.ru

Or with thin:

    bundle exec thin -C thin.yml -R config.ru start

### Creating the Initial DB ###

This is mainly for my own documentation since it is to import from the old PHP
app:

    bubdle exec ruby quotes2db.rb quotes.db quotes/*.{profile,quotes}