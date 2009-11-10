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

      def user
        return false unless @user
        { :path => "/users/#{@user.username}" }
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
