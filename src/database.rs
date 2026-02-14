use anyhow::Result;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::time::Duration;

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect("sqlite::memory:")
            .await?;

        let db = Self { pool };
        db.run_migrations().await?;
        Ok(db)
    }

    async fn run_migrations(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                first_name TEXT,
                last_name TEXT,
                avatar_url TEXT,
                bio TEXT,
                role TEXT NOT NULL DEFAULT 'user',
                status TEXT NOT NULL DEFAULT 'active',
                email_verified INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_login_at TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS posts (
                id TEXT PRIMARY KEY,
                author_id TEXT NOT NULL,
                title TEXT NOT NULL,
                slug TEXT UNIQUE NOT NULL,
                content TEXT NOT NULL,
                excerpt TEXT,
                featured_image_url TEXT,
                status TEXT NOT NULL DEFAULT 'draft',
                visibility TEXT NOT NULL DEFAULT 'public',
                tags TEXT,
                categories TEXT,
                view_count INTEGER NOT NULL DEFAULT 0,
                like_count INTEGER NOT NULL DEFAULT 0,
                comment_count INTEGER NOT NULL DEFAULT 0,
                published_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (author_id) REFERENCES users(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                sku TEXT UNIQUE NOT NULL,
                name TEXT NOT NULL,
                slug TEXT UNIQUE NOT NULL,
                description TEXT NOT NULL,
                short_description TEXT,
                price REAL NOT NULL,
                sale_price REAL,
                cost_price REAL,
                currency TEXT NOT NULL DEFAULT 'USD',
                quantity INTEGER NOT NULL DEFAULT 0,
                low_stock_threshold INTEGER NOT NULL DEFAULT 10,
                weight REAL,
                dimensions TEXT,
                images TEXT,
                thumbnail_url TEXT,
                category_id TEXT,
                brand_id TEXT,
                status TEXT NOT NULL DEFAULT 'active',
                is_featured INTEGER NOT NULL DEFAULT 0,
                is_digital INTEGER NOT NULL DEFAULT 0,
                meta_title TEXT,
                meta_description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS orders (
                id TEXT PRIMARY KEY,
                order_number TEXT UNIQUE NOT NULL,
                customer_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                payment_status TEXT NOT NULL DEFAULT 'pending',
                fulfillment_status TEXT NOT NULL DEFAULT 'unfulfilled',
                subtotal REAL NOT NULL,
                tax_amount REAL NOT NULL DEFAULT 0,
                shipping_amount REAL NOT NULL DEFAULT 0,
                discount_amount REAL NOT NULL DEFAULT 0,
                total REAL NOT NULL,
                currency TEXT NOT NULL DEFAULT 'USD',
                billing_address TEXT NOT NULL,
                shipping_address TEXT NOT NULL,
                shipping_method TEXT,
                tracking_number TEXT,
                notes TEXT,
                metadata TEXT,
                placed_at TEXT NOT NULL,
                paid_at TEXT,
                shipped_at TEXT,
                delivered_at TEXT,
                cancelled_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (customer_id) REFERENCES users(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS order_items (
                id TEXT PRIMARY KEY,
                order_id TEXT NOT NULL,
                product_id TEXT NOT NULL,
                variant_id TEXT,
                sku TEXT NOT NULL,
                name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                unit_price REAL NOT NULL,
                total_price REAL NOT NULL,
                tax_amount REAL NOT NULL DEFAULT 0,
                discount_amount REAL NOT NULL DEFAULT 0,
                metadata TEXT,
                FOREIGN KEY (order_id) REFERENCES orders(id),
                FOREIGN KEY (product_id) REFERENCES products(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                token_hash TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
