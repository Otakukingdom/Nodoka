//
// Created by mistlight on 1/8/17.
//

#include "Setting.h"

Core::Setting::Setting() {
    this->setting = new QSettings("nodoka.ini", QSettings::IniFormat);
}

void Core::Setting::setVolume(int volume) {
    this->setting->setValue("volume", volume);

    emit this->volumeUpdated(volume);
}

void Core::Setting::setCurrentAudiobook(int audiobookId) {
    this->setting->setValue("audiobook_id", audiobookId);
}

int Core::Setting::getVolume() {
    return this->setting->value("volume", 100).toInt();
}

int Core::Setting::getCurrentAudiobookId() {
    if(!this->setting->contains("audiobook_id")) {
        return -1;
    }

    return this->setting->value("audiobook_id").toInt();
}

