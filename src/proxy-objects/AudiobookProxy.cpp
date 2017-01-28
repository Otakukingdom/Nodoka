//
// Created by mistlight on 1/27/2017.
//

#include "AudiobookProxy.h"

AudiobookProxy::AudiobookProxy(QSqlRecord record, Core::Setting *settings) {
    this->record = record;
    this->settings = settings;

    auto idValue = record.value("id");
    auto directoryValue = record.value("directory");

    if(idValue.isNull() || directoryValue.isNull()) {
        this->isNull = true;
    } else {
        this->isNull = false;

        auto idStrValue = idValue.toString();
        auto directoryStrValue = directoryValue.toString();
        auto stringToHash = "Audiobook:" + idStrValue + directoryStrValue;

        auto path = Core::getUniqueSettingPath(stringToHash);
        this->currentFileSetting = QSharedPointer<QSettings>(new QSettings(path));
    }
}


void AudiobookProxy::remove() {

}

void AudiobookProxy::rescan() {

}

