//
// Created by mistlight on 1/27/2017.
//

#include "AudiobookProxy.h"
#include <QDebug>

AudiobookProxy::AudiobookProxy(QSqlRecord record, Core::Setting *settings) {
    this->record = record;
    this->settings = settings;

    auto idValue = record.value("id");
    auto directoryValue = record.value("directory");

    if(idValue.isNull() || directoryValue.isNull()) {
        this->isNull = true;
    } else {
        this->isNull = false;

        this->id = idValue.toString();
        this->directory = directoryValue.toString();

        auto stringToHash = "Audiobook:" + this->id + ":" + this->directory;

        auto path = Core::getUniqueSettingPath(stringToHash);
        this->currentFileSetting = QSharedPointer<QSettings>(new QSettings(path, QSettings::IniFormat));
    }
}


void AudiobookProxy::remove() {
    QString queryString = "DELETE FROM audiobooks WHERE id = ?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(this->id);
    if(query.exec()) {
        QFile::remove(this->currentFileSetting->fileName());

        emit this->removed();
        qDebug() << "Audiobook Removed";
    } else {
        qDebug() << "Audiobook Failed to be Removed";
    }
}

void AudiobookProxy::rescan() {

}

QAction* AudiobookProxy::getRemoveAction() {
    auto action = new QAction("Remove Audiobook");
    connect(action, &QAction::triggered, this, &AudiobookProxy::remove);
    return action;
}

