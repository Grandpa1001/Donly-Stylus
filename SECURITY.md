# 🔒 Security Guidelines

## ⚠️ Important Security Notes

### Private Keys and Sensitive Data
- **NEVER commit private keys to Git**
- **NEVER share private keys in public repositories**
- **Use environment variables for sensitive data**

### Files to Keep Private
- `.env` - Environment variables
- `.env.local` - Local environment variables
- `deployment-config.json` - Deployment configuration
- `*.key` - Private key files
- `wallet.json` - Wallet files
- `keystore/` - Keystore directories

### Environment Variables
Copy `env.example` to `.env` and fill in your values:

```bash
cp env.example .env
```

### Frontend Environment Variables
For frontend, create `.env.local`:

```bash
# In donly-frontend/
cp .env.example .env.local
```

### Deployment
1. **Never commit deployment configs with real keys**
2. **Use environment variables for deployment**
3. **Keep private keys in secure locations**
4. **Use different keys for different environments**

### Best Practices
- Use hardware wallets for production
- Rotate keys regularly
- Monitor for unauthorized access
- Use multi-signature wallets for important contracts
- Keep backups of important data in secure locations

## 🚨 If You Accidentally Commit Sensitive Data

1. **Immediately rotate the compromised key**
2. **Remove the data from Git history**
3. **Update all systems using the old key**
4. **Monitor for unauthorized activity**

```bash
# Remove sensitive file from Git history
git filter-branch --force --index-filter \
  'git rm --cached --ignore-unmatch sensitive-file.txt' \
  --prune-empty --tag-name-filter cat -- --all
```
