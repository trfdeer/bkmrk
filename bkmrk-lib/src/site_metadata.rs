use std::fmt::Display;

use eyre::{Result, WrapErr};

#[derive(Debug, Default, Clone)]
pub struct SiteMetadata {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub site_type: SiteType,
}

impl SiteMetadata {
    pub fn get_metadata(url: &str) -> Result<Self> {
        let info = webpage::Webpage::from_url(
            url,
            webpage::WebpageOptions {
                allow_insecure: true,
                ..Default::default()
            },
        )
        .wrap_err("Couldn't read from URL")?;

        let title = info.html.title.unwrap_or("".into());
        let description = info.html.description;
        let site_type = SiteType::from(&info.html.opengraph.og_type);
        let image_url = info
            .html
            .opengraph
            .images
            .iter()
            .map(|obj| obj.url.to_owned())
            .next();
        Ok(Self {
            title,
            description,
            image_url,
            site_type,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SiteType {
    MusicSong,
    MusicAlbum,
    MusicPlaylist,
    MusicRadioStation,
    VideoMovie,
    VideoEpisode,
    VideoTvShow,
    VideoOther,
    Article,
    Book,
    Profile,
    Website,
}

impl Default for SiteType {
    fn default() -> Self {
        Self::Website
    }
}

impl Display for SiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SiteType::MusicSong => write!(f, "music.song"),
            SiteType::MusicAlbum => write!(f, "music.album"),
            SiteType::MusicPlaylist => write!(f, "music.playlist"),
            SiteType::MusicRadioStation => write!(f, "music.radio_station"),
            SiteType::VideoMovie => write!(f, "video.movie"),
            SiteType::VideoEpisode => write!(f, "video.episode"),
            SiteType::VideoTvShow => write!(f, "video.tv_show"),
            SiteType::VideoOther => write!(f, "video.other"),
            SiteType::Article => write!(f, "article"),
            SiteType::Book => write!(f, "book"),
            SiteType::Profile => write!(f, "profile"),
            SiteType::Website => write!(f, "website"),
        }
    }
}

impl SiteType {
    pub fn from(txt: &str) -> Self {
        match txt {
            "music.song" => Self::MusicSong,
            "music.album" => Self::MusicAlbum,
            "music.playlist" => Self::MusicPlaylist,
            "music.radio_station" => Self::MusicRadioStation,
            "video.movie" => Self::VideoMovie,
            "video.episode" => Self::VideoEpisode,
            "video.tv_show" => Self::VideoTvShow,
            "video.other" => Self::VideoOther,
            "article" => Self::Article,
            "book" => Self::Book,
            "profile" => Self::Profile,
            "website" | _ => Self::Website,
        }
    }
}
