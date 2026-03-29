'use client';

import { useContext } from 'react';
import { ToastContext, ToastContextValue } from '@/providers/ToastProvider';

export function useToast(): ToastContextValue {
  const context = useContext(ToastContext);
  
  if (!context) {
    // Return dummy functions if used outside provider (e.g. during SSR/prerender)
    return {
      showInfo: () => {},
      showSuccess: () => {},
      showWarning: () => {},
      showError: () => {},
      removeToast: () => {},
      toasts: [],
    };
  }
  
  return context;
}
