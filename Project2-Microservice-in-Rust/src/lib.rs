/*A library that returns back a random Kanye West song */

use rand::Rng;

//create an const array of Top 10 Kanye West songs
const KANYE_SONGS: [&str; 10] = [
    "Gold Digger",
    "Stronger",
    "All Falls Down",
    "Heartless",
    "Jesus Walks",
    "Diamonds From Sierra Leone",
    "Flashing Lights",
    "Good Life",
    "Can't Tell Me Nothing",
    "Love Lockdown",
];

//create a function that returns a random fruit
pub fn random_kanye() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..KANYE_SONGS.len());
    KANYE_SONGS[random_index]
}
