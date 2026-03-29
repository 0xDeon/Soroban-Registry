import type { AppProps } from 'next/app';
import ToastProvider from '@/providers/ToastProvider';

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ToastProvider>
      <Component {...pageProps} />
    </ToastProvider>
  );
}
