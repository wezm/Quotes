require './view_helpers'

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

        def include_quotee
          false
        end

        def current_user
          @user == @current_user
        end

        def quotes
          @quotes.map do |q|
            {
              :quote_id => q.id,
              :body => q.quote_body,
              :created_at => formatted_date(q.created_at),
              :poster  => q.poster.username,
            }
          end
        end

        def pager
          @quotes.pager.to_html("/users/#{@user.username}")
        end
      end

    end

end
