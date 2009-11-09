require 'view_helpers'

module Quotes

    module Views

      class User < Mustache
        include Quotes::ViewHelpers

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

        def pager
          @quotes.pager.to_html("/users/#{@user.username}")
        end
      end

    end

end
