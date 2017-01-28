//
// Created by mistlight on 1/27/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKPROXY_H
#define NODOKANATIVE_AUDIOBOOKPROXY_H

#include <QFile>
#include <QSqlRecord>
#include <QSqlQuery>
#include <memory>
#include <set>
#include <src/core/Setting.h>
#include <src/core/Util.h>
#include <QSharedPointer>
#include <QSettings>

#include <QAction>


/**
 * AudiobookEvent are events that can potentially be initiated from this
 */
enum AudiobookEvent {
    Removed,
    Added
};

/**
 * Proxy object for Audiobook implements the proxy pattern. The real Audiobook is stored somewhere
 * in the database, and some of the frequently accessed data related to the Audiobook is stored
 * on the filesystem in a git-like hash object.
 *
 * The hash object is stored in a very similar fashion on how git objects are stored on the database.
 */
class AudiobookProxy : public QObject {
    Q_OBJECT

    QSharedPointer<QSettings> currentFileSetting;
    Core::Setting* settings;
    QSqlRecord record;
    bool isNull;

    // attribute
    QString id;
    QString directory;

    // callback functions
    std::map<AudiobookEvent, std::vector<std::function<void ()> > > callbackLookupTable;

    // callback set to keep track if we have added the callback function or not
    std::set<std::string> callbackFunctionList;

    void notifyCallbacks(AudiobookEvent event);


public:
    AudiobookProxy(QSqlRecord record, Core::Setting* settings);
    QAction* getRemoveAction();

    /**
     *
     * @param callbackType The type of event that will cause this callback to be initiated
     * @param callbackName A unique idenfier identifying the callback
     * @param callbackFunction
     */
    void addCallback(AudiobookEvent callbackType,
                     std::string callbackName,
                     std::function<void ()> callbackFunction);

public slots:
    void remove();
    void rescan();


};


#endif //NODOKANATIVE_AUDIOBOOKPROXY_H
