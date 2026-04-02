pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tickets = vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ];
        assert_eq!(find_itinerary(tickets), vec!["JFK","MUC","LHR","SFO","SJC"]);
    }

    #[test]
    fn single_ticket() {
        let tickets = vec![vec!["JFK".to_string(), "ATL".to_string()]];
        assert_eq!(find_itinerary(tickets), vec!["JFK", "ATL"]);
    }
}
