require 'view_helpers'

module Quotes

    module Views
      
      class Index < Mustache
        include Quotes::ViewHelpers

        def initialize(ssl = false)
          @ssl = ssl
        end

        def users
          @users.map do |u|
            {
              :name => u.username,
              :avatar => gravatar(u.email, 50),
              :user_path => "/users/#{u.username}"
            }
          end
        end

        def include_quotee
          true
        end

        def quotes
          @quotes.map do |q|
            {
              :quote_id => q.id,
              :body => q.quote_body,
              :created_at => formatted_date(q.created_at),
              :quotee => q.user.username,
              :poster  => q.poster.username,
            }
          end
        end

        def pager
          @quotes.pager.to_html('/')
        end

      end

    end

end
