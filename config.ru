require 'quotes'

# use Rack::Session::Pool
# openid_options = {
# }
# use Rack::Auth::OpenIDAuth, 'http://quotes.wezm.net/', openid_options do |env|
#   #env['rack.session'][:authkey] == a_string
#   # env['rack.session'][:user] = user.id
#   true
# end

run Quotes::App
