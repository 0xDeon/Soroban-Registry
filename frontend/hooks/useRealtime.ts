'use client';

import { useContext } from 'react';
import { RealtimeContext } from '@/providers/RealtimeProvider';

export function useRealtime() {
  const context = useContext(RealtimeContext);

  if (context) {
    return context;
  }

  return {
    isConnected: false,
    unreadCount: 0,
    notifications: [],
    subscribe: () => () => undefined,
    clearNotifications: () => undefined,
    markAsRead: () => undefined,
  };
}
