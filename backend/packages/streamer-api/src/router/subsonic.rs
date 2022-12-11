use axum::{
    body::{Body, HttpBody},
    extract::State,
    middleware::from_fn,
    routing::{get, MethodRouter},
    Json, Router,
};
use common::AppState;

use crate::{handler::subsonic::system::ping, middleware::subsonic::auth};

trait SubsonicRoute<S, B> {
    fn subsonic_route(self, path: &str, method_router: MethodRouter<S, B>) -> Self;
}

impl<S, B> SubsonicRoute<S, B> for Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn subsonic_route(self, path: &str, method_router: MethodRouter<S, B>) -> Self {
        self.route(path, method_router.clone())
            .route(&format!("{path}.view"), method_router)
    }
}

pub fn subsonic() -> Router<AppState, Body> {
    let authed_router = Router::new()
        // System
        .subsonic_route("/ping", get(ping))
        .subsonic_route("/getLicense", get(top_page))
        // Browse
        .subsonic_route("/getMusicFolders", get(top_page))
        .subsonic_route("/getIndexes", get(top_page))
        .subsonic_route("/getMusicDirectory", get(top_page))
        .subsonic_route("/getGenres", get(top_page))
        .subsonic_route("/getArtists", get(top_page))
        .subsonic_route("/getArtist", get(top_page))
        .subsonic_route("/getAlbum", get(top_page))
        .subsonic_route("/getSong", get(top_page))
        .subsonic_route("/getVideos", get(top_page))
        .subsonic_route("/getVideoInfo", get(top_page))
        .subsonic_route("/getArtistInfo", get(top_page))
        .subsonic_route("/getArtistInfo2", get(top_page))
        .subsonic_route("/getAlbumInfo", get(top_page))
        .subsonic_route("/getAlbumInfo2", get(top_page))
        .subsonic_route("/getSimilarSongs", get(top_page))
        .subsonic_route("/getSimilarSongs2", get(top_page))
        .subsonic_route("/getTopSongs", get(top_page))
        // Album/song lists
        .subsonic_route("/getAlbumList", get(top_page))
        .subsonic_route("/getAlbumList2", get(top_page))
        .subsonic_route("/getRandomSongs", get(top_page))
        .subsonic_route("/getSongsByGenre", get(top_page))
        .subsonic_route("/getNoePlaying", get(top_page))
        .subsonic_route("/getStarred", get(top_page))
        .subsonic_route("/getStarred2", get(top_page))
        // Searching
        .subsonic_route("/search", get(top_page))
        .subsonic_route("/search2", get(top_page))
        .subsonic_route("/search3", get(top_page))
        // Playlists
        .subsonic_route("/getPlaylists", get(top_page))
        .subsonic_route("/getPlatlist", get(top_page))
        .subsonic_route("/createPlaylist", get(top_page))
        .subsonic_route("/updatePlaylist", get(top_page))
        .subsonic_route("/deletePlaylist", get(top_page))
        // Media retrieval
        .subsonic_route("/stream", get(top_page))
        .subsonic_route("/download", get(top_page))
        .subsonic_route("/getCaptions", get(top_page))
        .subsonic_route("/getCoverArt", get(top_page))
        .subsonic_route("/getLyrics", get(top_page))
        .subsonic_route("/getAvatar", get(top_page))
        // Media annotation
        .subsonic_route("/star", get(top_page))
        .subsonic_route("/unstar", get(top_page))
        .subsonic_route("/setRating", get(top_page))
        .subsonic_route("/scrobble", get(top_page))
        // Sharing
        .subsonic_route("/getShares", get(top_page))
        .subsonic_route("/createShare", get(top_page))
        .subsonic_route("/updateShare", get(top_page))
        .subsonic_route("/deleteShare", get(top_page))
        // Podcast
        .subsonic_route("/getPodcasts", get(top_page))
        .subsonic_route("/getNewestPodcasts", get(top_page))
        .subsonic_route("/refreshPodcasts", get(top_page))
        .subsonic_route("/createPodcastChannel", get(top_page))
        .subsonic_route("/deletePodcastChannel", get(top_page))
        .subsonic_route("/deletePodcastEpisode", get(top_page))
        .subsonic_route("/downloadPodcastEpisode", get(top_page))
        // Jukebox
        .subsonic_route("/jukeboxControl", get(top_page))
        // Internet radio
        .subsonic_route("/getInternetRadioStations", get(top_page))
        .subsonic_route("/createInternetRadioStation", get(top_page))
        .subsonic_route("/updateInternetRadioStation", get(top_page))
        .subsonic_route("/deleteInternetRadioStation", get(top_page))
        // Chat
        .subsonic_route("/getChatMessages", get(top_page))
        .subsonic_route("/addChatMessage", get(top_page))
        // Bookmarks
        .subsonic_route("/getBookmarks", get(top_page))
        .subsonic_route("/createBookmark", get(top_page))
        .subsonic_route("/deleteBookmark", get(top_page))
        .subsonic_route("/getPlayQueue", get(top_page))
        .subsonic_route("/savePlayQueue", get(top_page))
        .layer(from_fn(auth));

    let admin_authed_router = Router::new()
        // User management
        .subsonic_route("/getUser", get(top_page))
        .subsonic_route("/getUsers", get(top_page))
        .subsonic_route("/createUser", get(top_page))
        .subsonic_route("/updateUser", get(top_page))
        .subsonic_route("/deleteUser", get(top_page))
        .subsonic_route("/changePassword", get(top_page))
        // Media library scanning
        .subsonic_route("/getScanStatus", get(top_page))
        .subsonic_route("/startScan", get(top_page));

    Router::new()
        .merge(authed_router)
        .merge(admin_authed_router)
}

async fn top_page(State(state): State<AppState>) -> Json<String> {
    Json("Hello, world!".to_string())
}
