export const cognitoConfig = {
  authority: 'https://cognito-idp.us-east-1.amazonaws.com/us-east-1_RIfjoPiFg',
  client_id: '79etm593hkt3a1f9jftuptubap',
  redirect_uri: 'http://localhost:1420/auth/callback',
  logout_uri: 'http://localhost:1420/auth/logout',
  domain: 'auth.seafoodfry.ninja',
};

export const getLogoutUrl = () =>
  `https://${cognitoConfig.domain}/logout?client_id=${cognitoConfig.client_id}&logout_uri=${encodeURIComponent(cognitoConfig.logout_uri)}`;
