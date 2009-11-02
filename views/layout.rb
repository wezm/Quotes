module Quotes
  module Views
    class Layout < Mustache
      def title
        @title || 'Quotes'
      end
      
      def flash_message
        return false unless @flash
        { :message => @flash }
      end

      def sidebar
        @sidebar || true
      end
    end
  end
end
