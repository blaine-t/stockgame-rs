use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
pub struct Stock {
    ticker: String,
    value: u64,
    volume: u64,
    volatility: f32,
    trend: f32,
}

// Balance options
static MIN_VALUE: u64 = 100;
static MAX_VALUE: u64 = 1000;

static MIN_VOLATILITY: f32 = -5.0;
static MAX_VOLATILITY: f32 = 5.0;

static MIN_TREND: f32 = -1.0;
static MAX_TREND: f32 = 1.0;

impl Stock {
    pub fn create_random() -> Self {
        Self::create(random_ticker())
    }

    pub fn create(ticker: String) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            ticker,
            value: rng.gen_range(MIN_VALUE..=MAX_VALUE),
            volume: 0,
            volatility: rng.gen_range(MIN_VOLATILITY..=MAX_VOLATILITY),
            trend: rng.gen_range(MIN_TREND..=MAX_TREND),
        }
    }
}

fn random_ticker() -> String {
    let mut rng = rand::thread_rng();

    let mut alphabet = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    alphabet.shuffle(&mut rng);

    alphabet.iter().take(3).collect()
}
