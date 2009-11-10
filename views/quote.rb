require 'view_helpers'

module Quotes

    module Views

      class Quote < Mustache
        include Quotes::ViewHelpers

        def initialize(ssl = false)
          @ssl = ssl
        end

        def name
          [@quote.user.firstname, @quote.user.surname].join(' ')
        end

        def avatar
          gravatar(@quote.user.email, 50)
        end

        def include_quotee
          false
        end

        def body
          @quote.quote_body
        end

        def created_at
          formatted_date(@quote.created_at)
        end

        def poster
          @quote.poster.username
        end

        def quotee
          @quote.user.username
        end
      end

    end

end
