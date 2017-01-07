//
// Created by mistlight on 12/22/2016.
//

#include "ConcretePlayer.h"

Core::ConcretePlayer::ConcretePlayer() {
    /* Load the VLC engine */
    this->inst = libvlc_new(0, NULL);

    return;
}

void Core::ConcretePlayer::loadMedia(QString path) {
    this->currentPath = path;

    this->mediaItem = libvlc_media_new_path(this->inst, path.toStdString().c_str());
    this->mediaPlayer = libvlc_media_player_new_from_media(this->mediaItem);
}

void Core::ConcretePlayer::releaseMedia() {
    libvlc_media_player_release(this->mediaPlayer);
}

void Core::ConcretePlayer::play() {
    libvlc_media_player_play(this->mediaPlayer);
}

