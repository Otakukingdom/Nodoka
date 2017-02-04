//
// Created by mistlight on 1/7/17.
//

#ifndef NODOKANATIVE_AUDIOBOOKRECORDPROXY_H
#define NODOKANATIVE_AUDIOBOOKRECORDPROXY_H


#include <QSqlRecord>
#include <QSqlQuery>
#include <QVariant>
#include <QHash>
#include <QSharedPointer>
#include <src/core/Setting.h>
#include "src/model/MediaProperty.h"
#include <QSettings>
#include <memory>

class AudiobookFileProxy {
    QSharedPointer<QSettings> currentFileSetting;
    Core::Setting* setting;
    QSqlRecord record;
    bool isNull;
    MediaProperty mediaProperty;

    bool durationFunctionSet = false;
    bool completenssFunctionSet = false;
    std::function<void ()> totalDurationUpdateFunction;
    std::function<void ()> totalCompletenessUpdateFunction;

    // calculate the hashsum for the current file and save it
    QString calcCheckSum();
    bool fileExistFlag;

public:
    AudiobookFileProxy(QSqlRecord record, Core::Setting* setting);
    AudiobookFileProxy();
    void calcAndWriteCheckSum(bool forced = false);
    QString path();
    QString name();
    bool getNullState();
    bool isPropertyParsed();
    void setAsCurrent();
    void saveCurrentTime(long long currentTime);
    void setAsComplete();
    long long getCurrentTime();
    bool currentTimeNull();
    int getCompleteness();
    bool fileExists();

    bool hasNextFile();
    AudiobookFileProxy getNextFile();
    QSqlRecord getRecord();
    void setTotalDurationUpdateFunction(std::function<void()> audiobookProxyUpdateFunction);
    void setCompletenessUpdateFunction(std::function<void()> func) ;

    void setProperty(MediaProperty property);
    void setMediaDuration(const long long duration);
    long long getMediaDuration();

    // direct user actions
    void resetReadStatus();
    void markAsRead();
    void remove();

};

#endif //NODOKANATIVE_AUDIOBOOKRECORDPROXY_H
