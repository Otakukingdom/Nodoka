//
// Created by mistlight on 12/22/2016.
//

#include <src/model/AudiobookListDelegate.h>
#include <src/proxy-objects/ProxyManager.h>
#include <QFileDialog>
#include "MainWindow.h"

const static int MAXIMUM_VOLUME = 150;

MainWindow::MainWindow(Directory* directoryModel,
                       Audiobook* audiobookModel,
                       Core::ConcretePlayer* player,
                       Core::ScanPlayer* scanPlayer,
                       Core::Setting* setting,
                       std::shared_ptr<ProxyManager> manager,
                       std::shared_ptr<AudiobookCollectionHandler> handler,
                       QWidget *parent) :
    QMainWindow(parent), ui(new Ui::MainWindow()) {
    ui->setupUi( this );
    this->setIsPlaying(false);

    this->settings = setting;

    // we will need this reference so FileList can make direct reference to it
    this->concretePlayer = player;
    this->scanPlayer = scanPlayer;

    // set the model
    this->directoryModel = directoryModel;
    this->audiobookModel = audiobookModel;

    // set up the event handlers
    this->abListHandler = new AudiobookListViewHandler(this,
                                                       this->ui->audiobookView,
                                                       this->ui->fileView,
                                                       manager,
                                                       this);
    this->fileListHandler = new FileListViewHandler(this,
                                                    this->ui->fileView,
                                                    manager,
                                                    this);
    this->collectionHandler = handler;

    // set up a null file
    this->currentlyPlayingFile = std::shared_ptr<AudiobookFileProxy>(new AudiobookFileProxy());

    this->fileDisplayModel = new FileDisplayModel(manager, this);

    // initialize the settings form
    this->settingsForm = new SettingsForm(this->directoryModel);
    this->settingsForm->hide();


    this->menuSetup();
    this->setup();
}

void MainWindow::menuSetup() {
    // create the actions we will use for the menu
    QAction* settings = new QAction("Add/Remove Scanning Directories");
    QAction* audiobookAdd = new QAction("Add an Audiobook");
    QAction* rescan = new QAction("Rescan Directories");

    // connect the actions to specific functions that will trigger its functionality
    connect(settings, &QAction::triggered, this, &MainWindow::performSettings);
    connect(rescan, &QAction::triggered, this, &MainWindow::performRescan);
    connect(audiobookAdd, &QAction::triggered, this, &MainWindow::performAudiobookAdd);

    // create the menu and add all of the actions
    this->audiobookMenu = new QMenu("Audiobook Menu", this->ui->abToolButton);
    this->audiobookMenu->addAction(audiobookAdd);
    this->audiobookMenu->addAction(rescan);
    this->audiobookMenu->addAction(settings);

    // connect the menu
    connect(this->ui->abToolButton, &QToolButton::clicked, [this]() {
        this->audiobookMenu->exec(
                this->ui->abToolButton->mapToGlobal(
                        QPoint(0, this->ui->abToolButton->height())));
    });


    // set up context menu for audiobookView
    this->ui->audiobookView->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(this->ui->audiobookView, &QWidget::customContextMenuRequested,
            this->abListHandler, &AudiobookListViewHandler::contextMenuRequested);

    // the same for file list view
    this->ui->fileView->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(this->ui->fileView, &QWidget::customContextMenuRequested,
            this->fileListHandler, &FileListViewHandler::contextMenuRequested
    );
}


void MainWindow::setup() {
    this->setWindowTitle("Nodoka");

    // set the initial label
    this->ui->currentlyPlayingLabel->setTextFormat(Qt::RichText);
    this->setLabel(this->ui->currentlyPlayingLabel);


    // populate the speed combo box
    this->populateSpeedChoose();


    // set up the audobook view
    auto audiobookListDelegate = new AudiobookListDelegate(AB_ITEM_STYLESHEET, 20);
    this->ui->audiobookView->setModel(this->audiobookModel);
    this->ui->audiobookView->setItemDelegate(audiobookListDelegate);
    this->ui->audiobookViewVertical->setStyleSheet(LIST_VIEW_STYLESHEET);
    this->ui->audiobookView->setFocusPolicy(Qt::NoFocus);
    this->ui->audiobookView->setSelectionMode(QAbstractItemView::SingleSelection);


    // connect the audiobook view events to the file selector view
    auto audiobookModel = this->audiobookModel;
    connect(this->ui->audiobookView->selectionModel(), &QItemSelectionModel::selectionChanged,
            [this, audiobookModel] (const QItemSelection &selected, const QItemSelection &deselected) {
                if(selected.indexes().size() > 0) {
                    // set the audiobook file list
                    auto modelIndex = selected.indexes().first();
                    auto record = audiobookModel->record(modelIndex.row());
                    int audiobookId = record.value("id").toInt();

                    this->fileDisplayModel->setSelectedAudiobook(audiobookId);

                    // set the selected audiobook file if it exists
                    QSqlQuery query;
                    query.prepare("SELECT id, selected_file from audiobooks WHERE id=?");
                    query.addBindValue(audiobookId);
                    if(!query.exec()) {
                        // if we are here, something went wrong while the query was executing
                        auto error = query.lastError();
                        qWarning() << "Something went wrong: "
                                   << error.driverText()
                                   << ", " << error.databaseText();
                    } else {
                        if(query.next()) {
                            // if we are here, it means the current file exists, and we are setting the selection
                            auto currentRecord = query.record();
                            QString path = "";
                            if(!currentRecord.value("selected_file").isNull()) {
                                path = currentRecord.value("selected_file").toString();
                                this->setSelectedFile(path);
                            } else {
                                // don't even proceed if this failed
                                return;
                            }

                            // check if the player does not have a media loaded, if the player does not have a media loaded
                            // it means the player hasn't loaded a file yet, we should load whichever file the user left off
                            // if that is the case...
                            if(this->concretePlayer->getAudiobookFile() == nullptr) {
                                auto index = this->fileDisplayModel->getFileIndex(path);
                                auto currentFileRecord = this->fileDisplayModel->record(index.row());

                                this->concretePlayer->loadMedia(currentFileRecord);
                            }
                        }
                    }

                }
            });

    // set up fileView
    auto fileListDelegate = new AudiobookListDelegate(FILE_ITEM_STYLESHEET, 10);
    this->ui->fileView->setModel(this->fileDisplayModel);
    this->ui->fileView->setItemDelegate(fileListDelegate);
    this->ui->fileViewVertical->setStyleSheet(LIST_VIEW_STYLESHEET);
    this->ui->fileView->setFocusPolicy(Qt::NoFocus);

    // connect file selector view to concrete player
    connect(this->ui->fileView, &QListView::doubleClicked,
            [this] (const QModelIndex &index) {
                QSqlTableModel* model = (QSqlTableModel *) this->ui->fileView->model();
                auto row = index.row();
                auto currentRecord = model->record(row);

                if(this->concretePlayer->canLoadMedia(currentRecord)) {
                    this->concretePlayer->releaseMedia();
                    this->concretePlayer->loadMedia(currentRecord);
                    this->concretePlayer->play();
                }
    });

    // prevent editing of audiobook and file list view
    this->ui->audiobookView->setEditTriggers(QAbstractItemView::NoEditTriggers);
    this->ui->fileView->setEditTriggers(QAbstractItemView::NoEditTriggers);

    // slider interaction is disabled by default
    this->ui->progressSlider->setEnabled(false);
    this->ui->progressSlider->setTracking(false);

    // define what the play button do
    connect(this->ui->playButton, &QPushButton::clicked, [=]() {
        if(this->isPlaying) {
            this->concretePlayer->stop();
        } else {
            this->concretePlayer->play();
        }
    });

    // connect the changes from the progress slider to the player
    connect(this->ui->progressSlider, &QSlider::sliderMoved,
            concretePlayer, &Core::ConcretePlayer::updateSeekPosition);

    connect(this->ui->progressSlider, &QSlider::sliderPressed,
            [=]() {
                this->concretePlayer->stop();
            });

    connect(this->ui->progressSlider, &QSlider::sliderReleased,
            [=]() {
                this->concretePlayer->play();
            });

    // set up the volume controls
    this->ui->volumeSlider->setMaximum(MAXIMUM_VOLUME);
    this->ui->volumeSlider->setValue(settings->getVolume());

    connect(this->ui->volumeSlider, &QSlider::sliderMoved,
            this->settings, &Core::Setting::setVolume);

    loadCurrentAudiobookIfExists();
}


MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::performExit() {
    this->close();
}

void MainWindow::performSettings() {
    this->settingsForm->setWindowModality(Qt::WindowModality::ApplicationModal);
    this->settingsForm->show();
}

void MainWindow::setIsPlaying(bool isPlaying) {
    this->isPlaying = isPlaying;

    if(!isPlaying) {
        QIcon playIcon(":/icons/play.png");
        this->ui->playButton->setIcon(playIcon);
    } else {
        QIcon playIcon(":/icons/pause.png");
        this->ui->playButton->setIcon(playIcon);
    }
}

void MainWindow::playerStateUpdated(std::shared_ptr<AudiobookFileProxy> abFile, bool isPlaying) {
    this->setCurrentlyPlayingFile(abFile);
    this->setIsPlaying(isPlaying);
}

void MainWindow::playerTimeUpdated(std::shared_ptr<AudiobookFileProxy> abFile, long long currentTime) {
    this->setCurrentTime(currentTime);
}

void MainWindow::setCurrentlyPlayingFile(std::shared_ptr<AudiobookFileProxy> file) {

    this->currentlyPlayingFile = file;

    this->setSelectedFile(file->path());

    if(this->currentlyPlayingFile->getNullState() == false) {
        setLabel(this->ui->currentlyPlayingLabel, this->currentlyPlayingFile);
    }

    // set the slider max value if we have a parsed duration
    if(this->currentlyPlayingFile->isPropertyParsed()) {
        long long totalDuration = this->currentlyPlayingFile->getMediaDuration();
        this->ui->progressSlider->setMaximum(static_cast<int>(totalDuration));

        // enable the slider
        this->ui->progressSlider->setEnabled(true);

        // init the slider with the saved current time
        if(!file->currentTimeNull()) {
            this->setCurrentTime(file->getCurrentTime());
        }
    }
}

void MainWindow::setCurrentTime(long long currentTime) {
    this->currentTime = currentTime;

    // update the progress slider
    this->ui->progressSlider->setValue(static_cast<int>(currentTime));

    this->setLabel(this->ui->currentlyPlayingLabel, this->currentlyPlayingFile, currentTime);

    // tell the file list view to update as well...
    this->ui->fileView->update();
}

// if there is an update with the AudiobookFile state, the Proxy file will be updated
void MainWindow::audiobookFileStateUpdated(std::shared_ptr<AudiobookFileProxy> abFile) {
    this->setCurrentlyPlayingFile(abFile);
}

void MainWindow::loadCurrentAudiobookIfExists() {
    auto audiobookId = this->settings->getCurrentAudiobookId();
    if(!audiobookId != -1) {
        Audiobook* currentModel = reinterpret_cast<Audiobook*>(this->ui->audiobookView->model());
        for(int i = 0; i < currentModel->rowCount(); i++) {
            if(currentModel->record(i).value("id").toInt() == audiobookId) {
                QModelIndex currentIndex = currentModel->index(i, 0);
                auto selectionModel = this->ui->audiobookView->selectionModel();
                selectionModel->select(currentIndex, QItemSelectionModel::Select);
                return;
            }
        }

    }
}

void MainWindow::setSelectedFile(QString path) {
    this->updateFileView();

    // deselect all first
    this->ui->fileView->selectionModel()->clearSelection();

    auto index =
            reinterpret_cast<FileDisplayModel*>(this->ui->fileView->model())->getFileIndex(path);

    this->ui->fileView->selectionModel()->select(index, QItemSelectionModel::Select);
}

void MainWindow::updateFileView() {
    auto model = static_cast<QSqlTableModel*>(this->ui->fileView->model());
    model->select();
}

void MainWindow::populateSpeedChoose() {
    this->ui->speedChooser->addItem("0.5x", QVariant("0.5"));
    this->ui->speedChooser->addItem("0.75x", QVariant("0.75"));
    this->ui->speedChooser->addItem("1x", QVariant("1"));
    this->ui->speedChooser->addItem("1.25x", QVariant("1.25"));
    this->ui->speedChooser->addItem("1.5x", QVariant("1.5"));
    this->ui->speedChooser->addItem("1.75x", QVariant("1.75"));
    this->ui->speedChooser->addItem("2x", QVariant("2"));
    this->ui->speedChooser->addItem("2.25x", QVariant("2.25"));
    this->ui->speedChooser->addItem("2.5x", QVariant("2.5"));

    auto speedFromSetting = this->settings->getSpeed();
    this->setSpeed(speedFromSetting);


    connect(this->ui->speedChooser,
            static_cast<void (QComboBox::*)(int)>(&QComboBox::currentIndexChanged),
            [this](int index) -> void {
        auto currentData = this->ui->speedChooser->itemData(index);
        auto speedString = currentData.toString();
        this->settings->setSpeed(speedString);

        this->concretePlayer->setSpeed(speedString);
    });
}

void MainWindow::setSpeed(QString speed) {
    int index = -1;
    for(int i = 0; i < this->ui->speedChooser->count(); i++) {
        auto currentData = this->ui->speedChooser->itemData(i);
        QString currentUserData = currentData.toString();

        if(currentUserData == speed) {
            index = i;
            break;
        }
    }

    if(index != -1) {
        this->ui->speedChooser->setCurrentIndex(index);
    }
}

void MainWindow::setLabel(QLabel *pLabel, std::shared_ptr<AudiobookFileProxy> proxy, long long currentTime) {
    QString text = "<div id=\"playing-label\">";
    if(proxy->getNullState()) {
        text += "<span style=\"font-size: 15px; font-style: italic;\">No File Loaded</span>";
    } else {
        text += "<span style=\"font-size: 15px; font-weight: bold;\">" + proxy->name() +"</span>";
    }

    text += "<br>";

    if(currentTime >= 0) {
        // update the label
        text += "<span>";
        text += Core::convertTimeToString(currentTime);
        text += "</span>";
    }

    text += "</div>";

    pLabel->setText(text);
}

// TODO perform rescan of audiobooks
void MainWindow::performRescan() {

}

void MainWindow::performAudiobookAdd() {
    auto target = QFileDialog::getExistingDirectory(this, "Select Folder", "", QFileDialog::ShowDirsOnly);

    // only perform this when user has actually selected something
    if(!target.isEmpty()) {
        this->collectionHandler->directoryAdded(target);
    }
}

