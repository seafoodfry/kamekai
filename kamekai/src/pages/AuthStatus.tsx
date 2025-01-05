import { useAuth } from 'react-oidc-context';
import { Button } from '@/components/ui/button';
import { Home, Loader2 } from 'lucide-react';
import { error, info } from '@tauri-apps/plugin-log';
import { getLogoutUrl } from '@/config/auth';
import { JWTDisplay } from '@/components/JWTDisplay';

interface AuthStatusProps {
  onBack?: () => void;
}

export const AuthStatus = ({ onBack }: AuthStatusProps) => {
  const auth = useAuth();

  // eslint-disable-next-line no-console
  console.log('AuthStatus current url:', window.location.href);
  // eslint-disable-next-line no-console
  console.log('auth navigator:', auth.activeNavigator);
  info(
    `Auth status: ${JSON.stringify({
      isAuthenticated: auth.isAuthenticated,
      activeNavigator: auth.activeNavigator,
      isLoading: auth.isLoading,
      error: auth.error,
    })}`
  );

  if (auth.isLoading) {
    return (
      <div className="flex items-center justify-center p-4">
        <Loader2 className="h-6 w-6 animate-spin text-blue-500" />
      </div>
    );
  }

  if (auth.error) {
    error(`Auth error: ${JSON.stringify(auth.error)}`);
    return <div className="text-red-500 p-4">Error: {auth.error.message}</div>;
  }

  if (auth.isAuthenticated) {
    return (
      <div className="flex flex-col gap-4 p-4">
        {onBack && (
          <Button
            onClick={onBack}
            variant="ghost"
            size="icon"
            className="absolute top-4 left-4 text-gray-400 hover:text-white hover:bg-[#2A2A2A]"
          >
            <Home className="h-5 w-5" />
          </Button>
        )}

        <div className="text-sm text-gray-400">Logged in as: {auth.user?.profile.email}</div>
        <JWTDisplay token={auth.user?.id_token} />

        <Button
          onClick={() => {
            auth.removeUser();
            // Invalidate the cognito session.
            window.location.href = getLogoutUrl();
          }}
          variant="outline"
          className="bg-red-600 hover:bg-red-700 text-white"
        >
          Sign Out
        </Button>
      </div>
    );
  }

  return (
    <div className="p-4">
      <Button
        onClick={() => auth.signinRedirect()}
        className="bg-blue-600 hover:bg-blue-700 text-white"
      >
        Sign In
      </Button>
    </div>
  );
};
