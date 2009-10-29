require 'quotes'

# use Rack::Session::Pool
# use Rack::Auth::OpenIDAuth, realm, openid_options do |env|
#   env['rack.session'][:authkey] == a_string
# end

run Quotes::App
