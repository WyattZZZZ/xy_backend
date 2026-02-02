use crate::database::models::{User, UserResponse};

pub struct SearchMatch {
    pub user: UserResponse,
    pub score: f32,
}

/// Search users with fuzzy matching and intelligent ranking
pub fn search_users(users: &[User], query: &str) -> Vec<UserResponse> {
    if query.trim().is_empty() {
        return users.iter()
            .map(|u| UserResponse::from(u.clone()))
            .collect();
    }

    let mut matches: Vec<SearchMatch> = users.iter()
        .filter_map(|user| {
            let score = calculate_user_score(user, query);
            if score > 0.0 {
                Some(SearchMatch {
                    user: UserResponse::from(user.clone()),
                    score,
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by score descending
    matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    
    matches.into_iter().map(|m| m.user).collect()
}

fn calculate_user_score(user: &User, query: &str) -> f32 {
    let query_lower = query.to_lowercase();
    let mut total_score = 0.0;

    // Search in name (weight: 10)
    total_score += search_field(&user.name, &query_lower, 10.0);
    
    // Search in bio (weight: 5)
    total_score += search_field(&user.bio, &query_lower, 5.0);
    
    // Search in location (weight: 5)
    total_score += search_field(&user.location, &query_lower, 5.0);
    
    // Search in email (weight: 3, prefix only)
    if user.email.to_lowercase().starts_with(&query_lower) {
        total_score += 3.0 * 10.0; // prefix match
    }

    total_score
}

fn search_field(field: &str, query: &str, weight: f32) -> f32 {
    let field_lower = field.to_lowercase();
    
    if field_lower.is_empty() || query.is_empty() {
        return 0.0;
    }

    // Prefix match (highest score)
    if field_lower.starts_with(query) {
        return weight * 10.0;
    }

    // Word boundary match
    if let Some(pos) = field_lower.find(query) {
        // Check if match is at word boundary
        if pos == 0 || field_lower.chars().nth(pos - 1).map_or(false, |c| c.is_whitespace()) {
            return weight * 7.0 / (pos as f32 + 1.0);
        }
        
        // Regular substring match
        return weight * 5.0 / (pos as f32 + 1.0);
    }

    0.0
}
