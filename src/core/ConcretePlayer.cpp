//
// Created by mistlight on 12/22/2016.
//

#include <QDebug>
#include <include/vlc/vlc.h>
#include "ConcretePlayer.h"

Core::ConcretePlayer::ConcretePlayer(Setting* setting) {
    // load settings
    this->setting = setting;

    // init volume based on the settings file
    this->volume = setting->getVolume();

    /* Load the VLC engine */
    this->inst = libvlc_new(0, NULL);
    this->mediaPlayer = libvlc_media_player_new(this->inst);
    this->playerEventManager = libvlc_media_player_event_manager(this->mediaPlayer);

    // set up callbacks
    this->setupVLCCallbacks();
    this->setupEventHandlers();

    // set the volume of the media player to the read volume
    libvlc_audio_set_volume(this->mediaPlayer, volume);

    // null initalization
    this->mediaLoaded = false;
    this->audiobookFileProxy = nullptr;
    this->autoPlay = false;

    this->hasSeekTo = false;
}

void Core::ConcretePlayer::loadMedia(QSqlRecord record) {
    if(this->mediaLoaded) {
        qDebug() << "Cannot load media if mediaLoaded is true";
        return;
    }

    this->audiobookFileProxy = std::shared_ptr<AudiobookFileProxy>(new AudiobookFileProxy(record, this->setting));
    this->currentPath = audiobookFileProxy->path();

    qDebug() << "Media read: " << this->currentPath;

    this->mediaItem = libvlc_media_new_path(this->inst, this->currentPath.toStdString().c_str());
    qDebug() << "Media item created : " << this->currentPath;
    libvlc_media_player_set_media(this->mediaPlayer, this->mediaItem);
    qDebug() << "Media loaded: " << this->currentPath;
    this->mediaEventManager = libvlc_media_event_manager(this->mediaItem);

    this->setupMediaCallbacks();

    this->mediaLoaded = true;

    if(this->autoPlay) {
        this->play();
    }
}

void Core::ConcretePlayer::releaseMedia() {
    qDebug() << "media release called";
    this->mediaLoaded = false;
    if(this->mediaLoaded) {
        libvlc_media_release(this->mediaItem);
        qDebug() << "media released";
    }
}

void Core::ConcretePlayer::play() {
    if(this->mediaLoaded) {
        libvlc_media_player_play(this->mediaPlayer);
        this->autoPlay = true;
    }
}

void Core::ConcretePlayer::stop() {
    libvlc_media_player_pause(this->mediaPlayer);
}

void Core::ConcretePlayer::setupVLCCallbacks() {

    libvlc_event_attach(this->playerEventManager,
                        libvlc_MediaPlayerTimeChanged,
                        (libvlc_callback_t) [](const struct libvlc_event_t * event, void *data) {
                            auto player = static_cast<ConcretePlayer*>(data);

                            if(player->mediaLoaded) {
                                emit player->timeProgressed(player->getCurrentTime());
                            }
                        },
                        this);

    libvlc_event_attach(this->playerEventManager,
                        libvlc_MediaPlayerEncounteredError,
                        (libvlc_callback_t) [](const struct libvlc_event_t * event, void *data) {
                            auto player = static_cast<ConcretePlayer*>(data);
                            auto errorMsg = libvlc_errmsg();

                            if(errorMsg != nullptr) {
                                qDebug() << "Error is: " << errorMsg;
                            }
                        },
                        this);
}

libvlc_state_t Core::ConcretePlayer::getCurrentState() {
    if(this->mediaPlayer == nullptr) {
        return libvlc_NothingSpecial;
    }

    return libvlc_media_player_get_state(this->mediaPlayer);
}

libvlc_time_t Core::ConcretePlayer::getCurrentTime() {
    return libvlc_media_player_get_time(this->mediaPlayer);
}

std::shared_ptr<AudiobookFileProxy> Core::ConcretePlayer::getAudiobookFile() {
    return this->audiobookFileProxy;
}

long long Core::ConcretePlayer::getDurationInMs() {
    long long durationInMs = libvlc_media_get_duration(this->mediaItem);
    if(durationInMs == -1) {
        return -1;
    }
    return durationInMs;
}

double Core::ConcretePlayer::getDurationInSeconds() {
    int durationInMs = libvlc_media_get_duration(this->mediaItem);
    if(durationInMs == -1) {
        return -1;
    }

    double durationInSeconds = durationInMs / 1000.0;

    return durationInSeconds;
}

void Core::ConcretePlayer::updateSeekPosition(long long position) {
    // first check if we have a file
    if(!this->mediaLoaded) {
        // do not bother if we don't have a loaded media
        return;
    }

    if(libvlc_media_player_is_seekable(this->mediaPlayer)) {
        libvlc_media_player_set_time(this->mediaPlayer, static_cast<libvlc_time_t>(position));
    } else {
        qDebug() << "Media not seekable";
    }
}

void Core::ConcretePlayer::setupEventHandlers() {
    connect(this->setting, &Setting::volumeUpdated, [=](int newVolume) {
        this->setVolume(newVolume);
    });
}

void Core::ConcretePlayer::setVolume(int volume) {
    this->volume = volume;

    if(this->mediaLoaded) {
        libvlc_audio_set_volume(this->mediaPlayer, volume);
    }
}

Core::ConcretePlayer::~ConcretePlayer() {
    qDebug() << "Player destructor called";
}

void Core::ConcretePlayer::setupMediaCallbacks() {
    libvlc_event_attach(this->mediaEventManager,
                        libvlc_MediaStateChanged,
                        (libvlc_callback_t) [](const struct libvlc_event_t * event, void *data) {
                            auto player = static_cast<ConcretePlayer*>(data);
                            auto newState = event->u.media_state_changed.new_state;
                            emit player->stateChanged(newState);
                        },
                        this);

    libvlc_event_attach(this->mediaEventManager,
                        libvlc_MediaParsedChanged,
                        (libvlc_callback_t) [](const struct libvlc_event_t * event, void *data) {
                            auto player = static_cast<ConcretePlayer*>(data);
                            if(!player->mediaLoaded) {
                                return;
                            }

                            int parsedStatus = libvlc_media_is_parsed(player->mediaItem);

                            if(parsedStatus) {
                                emit player->parsedStatusChanged(true);

                                // load the current time if possible
                                if(!player->audiobookFileProxy->currentTimeNull()) {
                                    player->updateSeekPosition(player->audiobookFileProxy->getCurrentTime());
                                }
                            } else {
                                emit player->parsedStatusChanged(false);
                            }
                        },
                        this);

    libvlc_event_attach(this->mediaEventManager,
                        libvlc_MediaFreed,
                        (libvlc_callback_t) [](const struct libvlc_event_t* event, void *data) {
                            auto player = static_cast<ConcretePlayer*>(data);

                            emit player->currentFileFinished();
                        }, this);


}
