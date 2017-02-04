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
#include <QMutex>

#include <QAction>
#include "AudiobookFileProxy.h"

class ProxyManager;

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
    std::function<std::shared_ptr<AudiobookFileProxy> (QSqlRecord record)> retrieveFileProxyFunction;
    QVector<std::shared_ptr<AudiobookFileProxy>> fileListCache;
    QMutex mutex;

    // attribute
    QString id;
    QString directory;

    // callback functions
    std::map<AudiobookEvent, std::vector<std::function<void ()> > > callbackLookupTable;

    // callback set to keep track if we have added the callback function or not
    std::set<std::string> callbackFunctionList;

    void notifyCallbacks(AudiobookEvent event);

    // internal function to load AudiobookFileProxy objects from database
    std::vector<std::shared_ptr<AudiobookFileProxy>> filesForAudiobookByDb(QString audiobookId,
                                                                           std::function<std::shared_ptr<AudiobookFileProxy>(QSqlRecord)>
                                                                           retrieveFileProxyFunction);

public:
    AudiobookProxy(QSqlRecord record,
                   Core::Setting* settings,
                   std::function<std::shared_ptr<AudiobookFileProxy> (QSqlRecord record)> retrieveFileProxyFunction);
    QAction* getRemoveAction();
    std::vector<std::shared_ptr<AudiobookFileProxy>> getFilesForAudiobook();

    /**
     *
     * @param callbackType The type of event that will cause this callback to be initiated
     * @param callbackName A unique idenfier identifying the callback
     * @param callbackFunction
     */
    void addCallback(AudiobookEvent callbackType,
                     std::string callbackName,
                     std::function<void ()> callbackFunction);

    bool hasDuration();
    bool allFileDurationScanned();
    long long getDuration();
    void setDuration(const long long duration);

    void handlePropertyScanFinished();

    void updateTotalDuration();
    void updateCompletionStatus();

    int getCompleteness();
    void resetReadStatus();

public slots:
    void remove();
    void rescan();


};


#endif //NODOKANATIVE_AUDIOBOOKPROXY_H
