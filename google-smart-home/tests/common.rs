pub fn two_way_serde<T: std::fmt::Debug + Eq + serde::ser::Serialize + serde::de::DeserializeOwned>(
    json: &str,
    expected: T,
) {
    {
        let parsed = serde_json::from_str::<T>(json).unwrap();
        assert_eq!(parsed, expected);
    }
    {
        let json = serde_json::to_string(&expected).unwrap();
        let parsed = serde_json::from_str::<T>(&json).unwrap();
        assert_eq!(parsed, expected);
    }
}

