'use client';

import { useContext } from 'react';
import { ToastContext, ToastContextValue } from '@/providers/ToastProvider';

export function useToast(): ToastContextValue {
  const context = useContext(ToastContext);

  if (context) {
    return context;
  }

  // Some prerendered routes can be evaluated before client-only providers mount.
  // In that case we expose a no-op toast API rather than crashing the build.
  return {
    toasts: [],
    showToast: () => undefined,
    dismissToast: () => undefined,
    showError: () => undefined,
    showSuccess: () => undefined,
    showWarning: () => undefined,
    showInfo: () => undefined,
  };
}
