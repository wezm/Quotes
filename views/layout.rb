module Quotes
  module Views
    class Layout < Mustache
      def title
        @title || 'Quotes'
      end
      
      def flash_message
        return false unless @flash_message
        { :message => @flash_message }
      end

      def flash_error
        return false unless @flash_error
        { :message => @flash_error }
      end

      def user
        return false unless @current_user
        { :path => "/users/#{@current_user.username}" }
      end

      def analytics
        !!@analytics_id
      end

      def analytics_id
        @analytics_id
      end
    end
  end
end
