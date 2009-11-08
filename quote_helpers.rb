module Quotes

  module QuoteHelpers
    def quotes
      @quotes.map do |q|
        created = q.created_at ? { :date => q.created_at.strftime('%a %d %b %Y %I:%M %p') } : false
        {
          :body => q.quote_body,
          :quotee => q.user.username,
          :created_at => created,
        }
      end
    end
  end

end
