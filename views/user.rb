require 'gravatar_helpers'
require 'quote_helpers'

module Quotes

    module Views

      class User < Mustache
        include Quotes::GravatarHelpers
        include Quotes::QuoteHelpers

        def initialize(ssl = false)
          @ssl = ssl
        end

        def avatar
          gravatar(@user.email, 50)
        end

        def username
          @user.username
        end

        def name
          [@user.firstname, @user.surname].join(' ')
        end

        def email
          @user.email
        end
      end

    end

end
