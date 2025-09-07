# 🚀 Deployment Guide - Donly Platform

## 📋 **Prerequisites**

- ✅ GitHub repository: `https://github.com/Grandpa1001/Donly-Stylus`
- ✅ Frontend folder: `donly-frontend/`
- ✅ Environment variables configured
- ✅ Firebase project set up
- ✅ Smart contract deployed on Arbitrum Sepolia

## 🌐 **Recommended: Vercel Deployment**

### **Why Vercel?**
- 🚀 **Built for Next.js** - Created by Next.js team
- 🔄 **Automatic deployments** - Deploy on every push
- 🌍 **Global CDN** - Fast loading worldwide
- 🔧 **Easy configuration** - Simple setup process
- 💰 **Free tier** - Perfect for hackathon projects

### **Step-by-Step Deployment**

#### **1. Connect to Vercel**
1. Go to [vercel.com](https://vercel.com)
2. Sign in with your GitHub account
3. Click **"New Project"**
4. Select **"Donly-Stylus"** repository
5. Choose **"donly-frontend"** as root directory

#### **1.1. Framework Settings (if not auto-detected)**
If Vercel doesn't auto-detect Next.js, configure manually:

- **Framework Preset**: `Next.js`
- **Root Directory**: `donly-frontend`
- **Build Command**: `npm run build`
- **Output Directory**: `.next`
- **Install Command**: `npm install`
- **Development Command**: `npm run dev`

**Note**: The project includes a `vercel.json` file in the root directory that automatically configures these settings for the `donly-frontend` subdirectory.

#### **2. Configure Environment Variables**
In Vercel dashboard, go to **Settings** → **Environment Variables** and add these variables:

```env
# Blockchain Configuration
NEXT_PUBLIC_ARBITRUM_SEPOLIA_RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
NEXT_PUBLIC_DONLY_CONTRACT_ADDRESS=0xb4e32dfc1c792424f57506a5113d40aae5fbc437
NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID=demo_project_id

# Firebase Configuration (Public)
NEXT_PUBLIC_FIREBASE_PROJECT_ID=donly-18a81
NEXT_PUBLIC_FIREBASE_PROJECT_NUMBER=583183369780
NEXT_PUBLIC_FIREBASE_API_KEY=AIzaSyDYmBw7JQmVIHWN_0Sytsc3Ulsle13v9do
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=donly-18a81.firebaseapp.com
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=donly-18a81.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=583183369780
NEXT_PUBLIC_FIREBASE_APP_ID=1:583183369780:web:8c605a5b845a4ed42fec61

# Firebase Admin SDK (Server-side only)
FIREBASE_ADMIN_PROJECT_ID=donly-18a81
FIREBASE_ADMIN_PRIVATE_KEY_ID=7cf2d178260dd669dc8d6dcc9db92505b608c366
FIREBASE_ADMIN_CLIENT_EMAIL=firebase-adminsdk-fbsvc@donly-18a81.iam.gserviceaccount.com
FIREBASE_ADMIN_CLIENT_ID=100997656680027851997
```

**Important**: Make sure to set these for **Production**, **Preview**, and **Development** environments.

#### **3. Deploy**
- Click **"Deploy"**
- Wait for build to complete (~2-3 minutes)
- Get your live URL: `https://donly-stylus.vercel.app`

#### **4. Automatic Updates**
- Every push to `main` branch = automatic deployment
- Preview deployments for pull requests
- Easy rollback to previous versions

## 🔧 **Alternative Deployment Options**

### **1. Netlify**
```bash
# Install Netlify CLI
npm install -g netlify-cli

# Build and deploy
cd donly-frontend
npm run build
netlify deploy --prod --dir=out
```

### **2. Railway**
1. Go to [railway.app](https://railway.app)
2. Connect GitHub repository
3. Select `donly-frontend` folder
4. Add environment variables
5. Deploy automatically

### **3. GitHub Pages (Static Export)**
```bash
# Add to next.config.js
const nextConfig = {
  output: 'export',
  trailingSlash: true,
  images: {
    unoptimized: true
  }
}

# Build and deploy
npm run build
# Upload 'out' folder to GitHub Pages
```

## 📊 **Deployment Checklist**

### **Before Deployment**
- [ ] All code committed to GitHub
- [ ] Environment variables ready
- [ ] Firebase project configured
- [ ] Smart contract deployed
- [ ] Build passes locally (`npm run build`)

### **After Deployment**
- [ ] Website loads correctly
- [ ] Wallet connection works
- [ ] Smart contract interactions work
- [ ] Firebase data loads
- [ ] All pages accessible

## 🔗 **Live Demo URLs**

Once deployed, your project will be available at:
- **Vercel**: `https://donly-stylus.vercel.app`
- **Netlify**: `https://donly-stylus.netlify.app`
- **Railway**: `https://donly-stylus.railway.app`

## 🎯 **Hackathon Presentation**

### **For ETH Warsaw 2025**
- **Live Demo**: Share the deployed URL
- **GitHub**: `https://github.com/Grandpa1001/Donly-Stylus`
- **Smart Contract**: `0xb4e32dfc1c792424f57506a5113d40aae5fbc437`
- **Network**: Arbitrum Sepolia

### **Demo Flow**
1. **Show live website** - Professional presentation
2. **Connect wallet** - Demonstrate Web3 integration
3. **Browse marketplace** - Show product listings
4. **Create campaign** - Demonstrate admin functions
5. **Purchase product** - Show payment flow
6. **View profile** - Show user management

## 🚨 **Troubleshooting**

### **Framework Detection Issues**

#### **Vercel doesn't detect Next.js**
If Vercel shows "No Framework Detected":

1. **Manual Configuration**:
   - Framework Preset: `Next.js`
   - Root Directory: `donly-frontend`
   - Build Command: `npm run build`
   - Output Directory: `.next`
   - Install Command: `npm install`
   - Development Command: `npm run dev`

2. **Alternative**: Use the `vercel.json` file (already created in project)

#### **Build Fails**
```bash
# Check for TypeScript errors
npm run build

# Fix any linting issues
npm run lint
```

#### **Environment Variables**
- Ensure all `NEXT_PUBLIC_` variables are set
- Check Firebase configuration
- Verify contract address

#### **Wallet Connection**
- Test on different browsers
- Ensure MetaMask is installed
- Check network configuration

## 📱 **Mobile Testing**

After deployment, test on mobile devices:
- **iOS Safari**: Test wallet connection
- **Android Chrome**: Test responsive design
- **Mobile MetaMask**: Test mobile wallet integration

## 🎉 **Success!**

Once deployed, you'll have:
- ✅ **Live website** accessible worldwide
- ✅ **Professional presentation** for hackathon
- ✅ **Automatic updates** on code changes
- ✅ **Fast loading** with global CDN
- ✅ **HTTPS security** for Web3 interactions

**Your Donly platform is now live and ready for ETH Warsaw 2025! 🚀**
