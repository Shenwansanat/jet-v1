import { BN } from '@project-serum/anchor';
import type { AccountInfo as TokenAccountInfo, MintInfo, u64 } from '@solana/spl-token';
import type { Market, AssetStore, Obligation } from '../models/JetTypes';
import { MARKET, ASSETS, DARK_THEME, WALLET } from '../store';

let wallet: any;
let market: Market | null;
let assets: AssetStore | null;
WALLET.subscribe(data => wallet = data);
MARKET.subscribe(data => market = data);
ASSETS.subscribe(data => assets = data);

// If user's browser has dark theme preference, set app to dark theme right on init
export const initDarkTheme = () => {
  let darkTheme: boolean = localStorage.getItem('jetDark') === 'true';
  if (darkTheme) {
    setDark(true);
  }
};

// Toggle dark theme root CSS attributes
export const setDark = (darkTheme: boolean): void => {
  if (darkTheme) {
    document.documentElement.style.setProperty('--black', '#ffffff');
    document.documentElement.style.setProperty('--grey', '#504f4f');
    document.documentElement.style.setProperty('--white', '#444444');
    document.documentElement.style.setProperty('--light-shadow', 'rgba(82, 82, 82, 1)');
    document.documentElement.style.setProperty('--dark-shadow', 'rgba(54, 54, 54, 1)');
    document.documentElement.style.setProperty('--input-color', 'rgba(255, 255, 255, 0.7)');
  } else {
    document.documentElement.style.setProperty('--black', '#1a495e');
    document.documentElement.style.setProperty('--grey', '#dee4ec');
    document.documentElement.style.setProperty('--white', '#e4ebf5');
    document.documentElement.style.setProperty('--light-shadow', 'rgba(255, 255, 255, 1)');
    document.documentElement.style.setProperty('--dark-shadow', 'rgba(190, 200, 228, 1)');
    document.documentElement.style.setProperty('--input-color', 'rgba(26, 73, 94, 0.7)');
  }

  localStorage.setItem('jetDark', JSON.stringify(darkTheme));
  DARK_THEME.set(darkTheme);
};

// Disconnect user wallet
export const disconnectWallet = () => {
  if (wallet.disconnect) {
    wallet.disconnect();
  }
  if (wallet.forgetAccounts) {
    wallet.forgetAccounts();
  }

  WALLET.set(null);
  ASSETS.set(null);
};

// Format USD or crypto with default or desired decimals
export const currencyFormatter = (value: number, usd: boolean, digits?: number) => {
  let currencyFormat: Intl.NumberFormat;
  let uiCurrency: string;
  if (usd) {
    currencyFormat = new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2, maximumFractionDigits: digits ?? 2 });
  } else {
    currencyFormat = new Intl.NumberFormat('en-US', { minimumFractionDigits: digits ?? 8, maximumFractionDigits: digits ?? 8 });
  }

  // Set and strip trailing 0's / unnecessary decimal if not USD
  uiCurrency = currencyFormat.format(value);
  if (!usd) {
    while (uiCurrency.indexOf('.') !== -1 && (uiCurrency[uiCurrency.length - 1] === '0' || uiCurrency[uiCurrency.length - 1] === '.')) {
      uiCurrency = uiCurrency.substring(0, uiCurrency.length - 1);
    }
  }

  return uiCurrency;
};

// Abbreviate large totals
export const totalAbbrev = (total: number, price?: number, native?: boolean, digits?: number): string => {
  let t = total;
  if (price && native === false) {
    t = total * price;
  }

  if (t > 1000000000) {
    return `${native ? '' : '$'}${(t / 1000000000).toFixed(1)}B`;
  } else if (t > 1000000) {
    return `${native ? '' : '$'}${(t / 1000000).toFixed(1)}M`;
  } else {
    return currencyFormatter(t, !native, native ? digits : 2);
  }
};

// Manual timeout promise to pause program execution
export const timeout = (ms: number): Promise<boolean> => {
  return new Promise((res) => {
    setTimeout(() => res(true), ms);
  });
};

// Calculate total value of deposits and borrowings, as well as c-ratio
export const getObligationData = (): Obligation => {
  let depositedValue: number = 0;
  let borrowedValue: number = 0;
  let colRatio = 0;
  let utilizationRate = 0;

  if (!assets || !market) {
    return {
      depositedValue,
      borrowedValue,
      colRatio,
      utilizationRate
    }
  }

  for (let t in assets.tokens) {
    depositedValue += new TokenAmount(
      assets.tokens[t].collateralBalance.amount,
      market.reserves[t].decimals
    ).uiAmountFloat * market.reserves[t].price;
    borrowedValue += new TokenAmount(
      assets.tokens[t].loanBalance.amount,
      market.reserves[t].decimals
    ).uiAmountFloat * market.reserves[t].price;

    colRatio = borrowedValue ? depositedValue / borrowedValue : 0;
    utilizationRate = depositedValue ? borrowedValue / depositedValue : 0;
  }

  return {
    depositedValue,
    borrowedValue,
    colRatio,
    utilizationRate
  }
};

// Token Amounts
export class TokenAmount {
  /** Raw amount of token lamports */
  public amount: BN;
  /** Number of decimals configured for token's mint */
  public decimals: number;
  /** Token amount as string, accounts for decimals */
  public uiAmount: string;
  /** Token amount as a float, accouts for decimals. Imprecise at large numbers */
  public uiAmountFloat: number;

  constructor(amount: BN, decimals: number) {
    if (!BN.isBN(amount)) {
      console.warn("Amount is not a BN", amount);
      amount = new BN(0);
    }
    this.amount = amount;
    this.decimals = decimals;
    this.uiAmount = TokenAmount.tokenAmount(amount, decimals);
    this.uiAmountFloat = parseFloat(this.uiAmount);
  }

  public static zero(decimals: number) {
    return new TokenAmount(new BN(0), decimals ?? 0);
  }

  public static tokenAccount(tokenAccount: TokenAccountInfo, decimals: number) {
    return new TokenAmount(tokenAccount.amount, decimals);
  }

  public static mint(mint: MintInfo) {
    return new TokenAmount(new BN(mint.supply), mint.decimals);
  }

  public static tokens(tokenAmount: string, decimals: number) {
    return new TokenAmount(TokenAmount.tokensToLamports(tokenAmount, decimals), decimals);
  }

  private static tokenAmount(lamports: BN, decimals: number) {
    let stringBN: string = lamports.toString();
    if (stringBN.length < decimals) {
      for (let i = 0; i <= decimals - stringBN.length; i++) {
        stringBN = '0' + stringBN;
      }
    }

    let num: string = [stringBN.slice(0, stringBN.length - decimals), '.', stringBN.slice(stringBN.length - decimals)].join('');
    if (!num.length) {
      num = '0';
    }

    // Format 100.000 = 100 cases
    while (num[num.length - 1] === '0') {
      num = num.slice(0, -1);
    }

    // Format 100. = 100 cases
    if (num[num.length - 1] === '.') {
      num = num.slice(0, -1);
    }

    return num.length ? num : '0';
  }

  // Convert a uiAmount string into lamports BN
  private static tokensToLamports(uiAmount: string, decimals: number) {
    let lamports: string = uiAmount;

    // Remove commas
    while (lamports.indexOf(',') !== -1) {
      lamports = lamports.replace(',', '');
    };

    // Determine if there's a decimal, take number of 
    // characters after it as fractionalValue
    let fractionalValue = 0;
    let initialPlace = lamports.indexOf('.');
    if (initialPlace !== -1) {
      fractionalValue = lamports.length - (initialPlace + 1);
      
      // If fractinoal value is lesser than a lamport, round to nearest lamport
      if (fractionalValue > decimals) {
        lamports = String(parseFloat(lamports).toFixed(decimals));
      }

      // Remove decimal
      lamports = lamports.replace('.', '');
    }

    // Append zeros
    for (let i = 0; i < decimals - fractionalValue; i++) {
      lamports += '0';
    }

    // Return BN value in lamports
    return new BN(lamports);
  };

  public add(b: TokenAmount) {
    return this.do(b, BN.prototype.add);
  }
  
  public addb(b: BN) { 
    return new TokenAmount(this.amount.add(b), this.decimals); 
  }

  public addn(b: number) {
    return new TokenAmount(this.amount.addn(b), this.decimals);
  }

  public sub(b: TokenAmount) {
    return this.do(b, BN.prototype.sub);
  }
  
  public subb(b: BN) { 
    return new TokenAmount(this.amount.sub(b), this.decimals); 
  }

  public subn(b: number) {
    return new TokenAmount(this.amount.subn(b), this.decimals);
  }

  public mul(b: TokenAmount) {
    return this.do(b, BN.prototype.mul);
  }
  
  public mulb(b: BN) { 
    return new TokenAmount(this.amount.mul(b), this.decimals); 
  }

  public muln(b: number) {
    return new TokenAmount(this.amount.muln(b), this.decimals);
  }

  public div(b: TokenAmount) {
    return this.do(b, BN.prototype.div);
  }
  
  public divb(b: BN) { 
    return new TokenAmount(this.amount.div(b), this.decimals); 
  }

  public divn(b: number) {
    return new TokenAmount(this.amount.divn(b), this.decimals);
  }

  private do(b: TokenAmount, fn: (b: BN) => BN) {
    if (this.decimals !== b.decimals) {
      console.warn("Decimal mismatch");
      return TokenAmount.zero(this.decimals);
    }
    let amount = fn.call(this.amount, b.amount);
    return new TokenAmount(amount, this.decimals);
  }
};

export type AmountUnits = { tokens?: {}, depositNotes?: {}, loanNotes?: {} };

export class Amount {
  private constructor(public units: AmountUnits, public value: BN) {}

  static tokens(amount: number | u64): Amount {
    return new Amount({ tokens: {} }, new BN(amount));
  }

  static depositNotes(amount: number | u64): Amount {
    return new Amount({ depositNotes: {} }, new BN(amount));
  }

  static loanNotes(amount: number | u64): Amount {
    return new Amount({ loanNotes: {} }, new BN(amount));
  }
}
