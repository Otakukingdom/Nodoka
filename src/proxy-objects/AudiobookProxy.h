//
// Created by mistlight on 1/27/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKPROXY_H
#define NODOKANATIVE_AUDIOBOOKPROXY_H

#include <QSqlRecord>
#include <src/core/Setting.h>
#include <src/core/Util.h>
#include <QSharedPointer>
#include <QSettings>


/**
 * Proxy object for Audiobook implements the proxy pattern. The real Audiobook is stored somewhere
 * in the database, and some of the frequently accessed data related to the Audiobook is stored
 * on the filesystem in a git-like hash object.
 *
 * The hash object is stored in a very similar fashion on how git objects are stored on the database.
 */
class AudiobookProxy {
    QSharedPointer<QSettings> currentFileSetting;
    Core::Setting* settings;
    QSqlRecord record;
    bool isNull;


public:
    AudiobookProxy(QSqlRecord record, Core::Setting* settings);
    void remove();
    void rescan();
};


#endif //NODOKANATIVE_AUDIOBOOKPROXY_H
