import { invoke } from "@tauri-apps/api/core";
import { Message } from "./message";

export class Session {
  constructor(public id: number, public name: string) {}

  /**
   * 获取当前会话的所有消息
   */
  public async list_messages(): Promise<Message[]> {
    return await invoke<Message[]>('get_all_messages', { sessionId: this.id });
  }

  /**
   * 删除当前会话
   */
  public async delete(): Promise<void> {
    return await invoke<void>('delete_session', { id: this.id });
  }

  /**
   * 更新当前会话的名称
   * @param newName 新的会话名称
   */
  public async update(newName: string): Promise<void> {
    return await invoke<void>('update_session', { id: this.id, name: newName });
  }

  /**
   * 在当前会话中添加一条消息
   * @param role 消息的角色
   * @param text 消息的文本内容
   * @param attachment_path 消息的附件路径
   */
  public async add_message(role: string, text?: string, attachment_path?: string): Promise<Message> {
    return await invoke<Message>('add_message', { session_id: this.id, role, text, attachment_path });
  }

  public static async create_session(name: string): Promise<Session> {
    const sessionData = await invoke<Session>('create_session', { name });
    return new Session(sessionData.id, sessionData.name);
  }

  public static async get_all_sessions(): Promise<Session[]> {
    const sessionDataList = await invoke<Session[]>('get_all_sessions');
    return sessionDataList.map(sessionData => new Session(sessionData.id, sessionData.name));
  }

  public static async get_session(id: number): Promise<Session> {
    const sessionData = await invoke<Session | null>('get_session', { id });
    if (sessionData === null) {
      throw new Error(`Session with id ${id} not found`);
    }
    return new Session(sessionData.id, sessionData.name);
  }
}