// (#)traits.rs 0.1.0   09/08/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::fmt::{Display, Formatter, Result};

/// Module that demonstrates traits in Rust

/// Summary trait
pub trait Summary {
    /// Summarize the item
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the item
    fn summarize(&self) -> String;

    /// Returns a summarizable item
    /// 
    /// # Returns
    /// 
    /// * `impl Summary` - A summarizable item
    fn returns_summarizable(&self) -> impl Summary;
    
    /// Categorize the item
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the item category
    fn categorize(&self) -> String {
        String::from("Setting the default category")
    }
}

/// News Article struct
/// 
/// # Fields
/// 
/// * `headline` - The headline of the news article
/// * `location` - The location of the news article
/// * `author` - The author of the news article
/// * `content` - The content of the news article
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// Implementation of Summary trait for NewsArticle
impl Summary for NewsArticle {
    /// Summarize the news article
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the news article
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    /// Returns a summarizable news article
    /// 
    /// # Returns
    /// 
    /// * `impl Summary` - A summarizable news article
    fn returns_summarizable(&self) -> impl Summary {
        NewsArticle {
            headline: self.headline.clone(),
            location: self.location.clone(),
            author: self.author.clone(),
            content: self.content.clone(),
        }
    }
}

/// Implementation of Display trait for News Article
impl Display for NewsArticle {
    /// Format the news article for display
    /// 
    /// # Arguments
    /// 
    /// * `f` - The formatter to write to
    /// 
    /// # Returns
    /// 
    /// * `Result` - A result indicating success or failure
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f, 
            "News Article - Headline: {}; Author: {}; Location: {}; Content: {}", 
            self.headline, 
            self.author, 
            self.location, 
            self.content
        )
    }
}

/// Social Post struct
/// 
/// # Fields
/// 
/// * `username` - The username of the social media user
/// * `content` - The content of the social media post
/// * `reply` - Whether the post is a reply
/// * `repost` - Whether the post is a repost
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

/// Implementation of Summary trait for SocialPost
impl Summary for SocialPost {
    /// Summarize the social media post
    /// 
    /// # Returns
    /// 
    /// * `String` - A string representation of the social media post
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    /// Returns a summarizable social media post
    /// 
    /// # Returns
    /// 
    /// * `impl Summary` - A summarizable social media post
    fn returns_summarizable(&self) -> impl Summary {
        SocialPost {
            username: self.username.clone(),
            content: self.content.clone(),
            reply: self.reply,
            repost: self.repost,
        }
    }
}

/// Implementation of Display trait for Social Post
impl Display for SocialPost {
    /// Format the social media post for display
    /// 
    /// # Arguments
    /// 
    /// * `f` - The formatter to write to
    /// 
    /// # Returns
    /// 
    /// * `Result` - A result indicating success or failure
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f, 
            "Social Post - Username: {}; Content: {}; Reply: {}; Repost: {}", 
            self.username, 
            self.content, 
            self.reply, 
            self.repost
        )
    }
}

/// The traits function
pub fn traits() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("{}", post.to_string());   // The to_string() method is available because we implemented the Display trait
    println!("New post available: {}", post.summarize());
    
    let summarizable_post = post.returns_summarizable();
    
    println!("Summarizable post: {}", summarizable_post.summarize());
    println!("Category: {}", post.categorize());

    notify(&post);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("{}", article);
    println!("New article available! {}", article.summarize());

    let summarizable_article = article.returns_summarizable();
    
    println!("Summarizable article: {}", summarizable_article.summarize());
    println!("Category: {}", article.categorize());
    
    notify(&article);
    
    let pair = Pair::new(1, 2);
    let strings = Pair::new(String::from("hello"), String::from("world"));

    pair.cmp_display();
    strings.cmp_display();
}

/// Notify function
/// Can be also written as: fn notify(item: &(impl Summary + Display))
/// As well as: fn notify<T>(item: &T) where T: Summary + Display
/// 
/// # Arguments
/// 
/// * `item` - A reference to an item that implements the Summary trait
/// 
/// # Returns
/// 
/// * `()` - No return value
fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/// Pair struct
/// 
/// # Type Parameters
/// 
/// * `T` - The type of the pair
struct Pair<T> {
    x: T,
    y: T,
}

/// Implementation of Pair struct
impl<T> Pair<T> {
    /// Create a new pair
    /// 
    /// # Arguments
    /// 
    /// * `x` - The first element of the pair
    /// * `y` - The second element of the pair
    /// 
    /// # Returns
    /// 
    /// * `Pair<T>` - A new pair
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// Implementation of Pair struct that only works with types that implement Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    /// Compare and display the largest member of the pair
    /// 
    /// # Arguments
    /// 
    /// * `self` - The pair to compare
    /// 
    /// # Returns
    /// 
    /// * `()` - No return value
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
